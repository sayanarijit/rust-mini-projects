trait Speaker {
    // Require implementations to define
    fn speak(&self) -> String;

    // Default implementation
    fn whisper(&self) -> String {
        String::from("pss! pss! pss!")
    }
}

trait Walker {
    fn walk(&self, from: i32, to: i32) {
        println!("walking from {} to {}", from, to);
    }
}

#[derive(Debug)]
struct Human {
    name: String,
}

impl Human {
    fn new(name: String) -> Human {
        Human { name }
    }
}

impl Speaker for Human {
    fn speak(&self) -> String {
        String::from("Yo man!")
    }
}

impl Walker for Human {}

fn speak(speaker: &impl Speaker) {
    println!("{}", &speaker.speak());
}

fn whisper(speaker: &impl Speaker) {
    println!("{}", speaker.whisper());
}

// Trait bound syntax
fn gossip<T: Speaker>(human1: &T, human2: &T) {
    whisper(human1);
    whisper(human2);
}

// Multiple trait bounds
fn present<T: Speaker + Walker>(presenter: &T) {
    for i in 1..30 {
        speak(presenter);
        presenter.walk(i % 5, (i + 1) % 5);
    }
}

// Multiple trait bounds with where clause
fn think<T>(thinker: &T)
where
    T: Walker + Speaker,
{
    for i in 1..30 {
        whisper(thinker);
        thinker.walk(i % 5, (i + 1) % 5);
    }
}

fn main() {
    let h1 = Human::new(String::from("foo"));
    println!("Hello, {:?}", &h1);
    speak(&h1);
    whisper(&h1);

    let h2 = Human::new(String::from("bar"));
    gossip(&h1, &h2);

    present(&h1);
    think(&h2);
}
