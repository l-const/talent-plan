use std::error::Error;

#[derive(Debug)]
struct ThreadPool {
    pool_size: u32,
}

trait Callable: FnOnce() + Send + 'static {}

trait IntoCallable {
    fn into_callable(self) -> Box<dyn FnOnce() + Send + 'static>;
}

#[derive(Debug)]
struct Job<F> where F: Callable {
    inner: F,
}

impl <F: Callable> IntoCallable for F {
    fn into_callable(self) -> Box<dyn FnOnce() + Send + 'static> {
        Box::new(self)
    }
}

impl <F: Callable> IntoCallable for Job<F> {
    fn into_callable(self) -> Box<dyn FnOnce() + Send + 'static> {
        Box::new(self.inner)
    }
}

impl<F: Callable> From<F> for Job<F> {
    fn from(f: F) -> Self {
        Self {
            inner: f,
        }
    }
}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

impl ThreadPool {
    pub fn new(threads: u32) -> Result<Self> {
        Ok(Self {pool_size: threads})
    }

    pub fn spawn<F>(&self, job: impl IntoCallable)  where F: Callable{
        std::thread::spawn(job.into_callable());
    }
}
