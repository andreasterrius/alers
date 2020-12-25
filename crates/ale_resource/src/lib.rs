use ale_autoid::{Identifiable, ProcessUniqueId};
use ale_error::PassedError;
use std::any::{Any, TypeId};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
