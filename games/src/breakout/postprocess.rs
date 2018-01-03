pub struct PostProcess {
    pub chaos : bool,
    pub shake : bool,
    pub confuse : bool
}

impl PostProcess {
    pub fn new() -> PostProcess {
        PostProcess {
            chaos: false,
            shake: false,
            confuse: false,
        }
    }
}