#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Credential {
    id: u64,
    student_id: u64,
    institution_id: u64,
    course: String,
    degree: String,
    graduation_year: u32,
    issued_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Institution {
    id: u64,
    name: String,
    address: String,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Student {
    id: u64,
    name: String,
    email: String,
    created_at: u64,
}

impl Storable for Credential {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Credential {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Institution {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Institution {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Student {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Student {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static CREDENTIALS_STORAGE: RefCell<StableBTreeMap<u64, Credential, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static INSTITUTIONS_STORAGE: RefCell<StableBTreeMap<u64, Institution, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static STUDENTS_STORAGE: RefCell<StableBTreeMap<u64, Student, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct CredentialPayload {
    student_id: u64,
    institution_id: u64,
    course: String,
    degree: String,
    graduation_year: u32,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct InstitutionPayload {
    name: String,
    address: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct StudentPayload {
    name: String,
    email: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct VerifyPayload {
    student_id: u64,
    institution_id: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
}

#[ic_cdk::update]
fn create_credential(payload: CredentialPayload) -> Result<Credential, Message> {
    if payload.course.is_empty() || payload.degree.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'course' and 'degree' are provided.".to_string(),
        ));
    }

    let student_exists = STUDENTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, student)| student.id == payload.student_id)
    });

    if !student_exists {
        return Err(Message::NotFound("Student not found".to_string()));
    }

    let institution_exists = INSTITUTIONS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, institution)| institution.id == payload.institution_id)
    });

    if !institution_exists {
        return Err(Message::NotFound("Institution not found".to_string()));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let credential = Credential {
        id,
        student_id: payload.student_id,
        institution_id: payload.institution_id,
        course: payload.course,
        degree: payload.degree,
        graduation_year: payload.graduation_year,
        issued_at: current_time(),
    };
    CREDENTIALS_STORAGE.with(|storage| storage.borrow_mut().insert(id, credential.clone()));
    Ok(credential)
}

#[ic_cdk::query]
fn get_credentials() -> Result<Vec<Credential>, Message> {
    CREDENTIALS_STORAGE.with(|storage| {
        let credentials: Vec<Credential> = storage
            .borrow()
            .iter()
            .map(|(_, credential)| credential.clone())
            .collect();

        if credentials.is_empty() {
            Err(Message::NotFound("No credentials found".to_string()))
        } else {
            Ok(credentials)
        }
    })
}

#[ic_cdk::query]
fn get_credential_by_id(id: u64) -> Result<Credential, Message> {
    CREDENTIALS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, credential)| credential.id == id)
            .map(|(_, credential)| credential.clone())
            .ok_or(Message::NotFound("Credential not found".to_string()))
    })
}

#[ic_cdk::update]
fn create_institution(payload: InstitutionPayload) -> Result<Institution, Message> {
    if payload.name.is_empty() || payload.address.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'name' and 'address' are provided.".to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let institution = Institution {
        id,
        name: payload.name,
        address: payload.address,
        created_at: current_time(),
    };
    INSTITUTIONS_STORAGE.with(|storage| storage.borrow_mut().insert(id, institution.clone()));
    Ok(institution)
}

#[ic_cdk::query]
fn get_institutions() -> Result<Vec<Institution>, Message> {
    INSTITUTIONS_STORAGE.with(|storage| {
        let institutions: Vec<Institution> = storage
            .borrow()
            .iter()
            .map(|(_, institution)| institution.clone())
            .collect();

        if institutions.is_empty() {
            Err(Message::NotFound("No institutions found".to_string()))
        } else {
            Ok(institutions)
        }
    })
}

#[ic_cdk::query]
fn get_institution_by_id(id: u64) -> Result<Institution, Message> {
    INSTITUTIONS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, institution)| institution.id == id)
            .map(|(_, institution)| institution.clone())
            .ok_or(Message::NotFound("Institution not found".to_string()))
    })
}

#[ic_cdk::update]
fn create_student(payload: StudentPayload) -> Result<Student, Message> {
    if payload.name.is_empty() || payload.email.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'name' and 'email' are provided.".to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let student = Student {
        id,
        name: payload.name,
        email: payload.email,
        created_at: current_time(),
    };
    STUDENTS_STORAGE.with(|storage| storage.borrow_mut().insert(id, student.clone()));
    Ok(student)
}

#[ic_cdk::query]
fn get_students() -> Result<Vec<Student>, Message> {
    STUDENTS_STORAGE.with(|storage| {
        let students: Vec<Student> = storage
            .borrow()
            .iter()
            .map(|(_, student)| student.clone())
            .collect();

        if students.is_empty() {
            Err(Message::NotFound("No students found".to_string()))
        } else {
            Ok(students)
        }
    })
}

#[ic_cdk::query]
fn get_student_by_id(id: u64) -> Result<Student, Message> {
    STUDENTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, student)| student.id == id)
            .map(|(_, student)| student.clone())
            .ok_or(Message::NotFound("Student not found".to_string()))
    })
}

#[ic_cdk::query]
fn verify_credential(payload: VerifyPayload) -> Result<Credential, Message> {
    CREDENTIALS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, credential)| {
                credential.student_id == payload.student_id
                    && credential.institution_id == payload.institution_id
            })
            .map(|(_, credential)| credential.clone())
            .ok_or(Message::NotFound("Credential not found".to_string()))
    })
}

fn current_time() -> u64 {
    time()
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    UnAuthorized { msg: String },
}

ic_cdk::export_candid!();
