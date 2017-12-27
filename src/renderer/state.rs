use renderer::job::RenderJob;
use std::collections::HashMap;
use math::*;

pub struct RenderState {
    pub last_frame : HashMap<i64, RenderJob>,
    pub current_frame : HashMap<i64, RenderJob>
}

impl RenderState {

    pub fn new() -> RenderState{
        RenderState{
            last_frame : HashMap::new(),
            current_frame : HashMap::new()
        }
    }

    pub fn interpolate_frame(&self, dt : f32) -> Vec<RenderJob>
    {
        use renderer::job::RenderJob::*;
        let mut renderjobs = vec!();

        //find everything in current frame that exists in last frame
        for (id, renderjob) in &self.current_frame {
            let lastjob_opt = self.last_frame.get(&id);
            if lastjob_opt.is_none(){
                continue;
            }
            let lastjob = lastjob_opt.unwrap();

            if let Some(renderjob) = self.lerp(&renderjob, &lastjob, dt) {
                renderjobs.push(renderjob);
            }
        }

        renderjobs
    }

    fn lerp(&self, a : &RenderJob, b : &RenderJob, dt : f32) -> Option<RenderJob> {
        use renderer::job::RenderJob::*;

        match(a, b){
            (&Sprite(ref at, ref ar), &Sprite(ref bt, ref br)) => Some(RenderJob::Sprite(at.lerp(&bt, dt), br.clone())),
            (&Particle(ref at, ref ar), &Particle(ref bt, ref br)) => Some(RenderJob::Particle(at.lerp(&bt, dt), br.clone())),
            _ => None
        }
    }
}
