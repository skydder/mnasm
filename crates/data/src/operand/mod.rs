mod register;
pub use register::{Register, RegisterKind};

mod path;
pub use path::Path;

mod immediate;
pub use immediate::Immediate;

mod memory;
pub use memory::{Memory, Scale};

mod string;
pub use string::Strings;
pub trait Operand {}
