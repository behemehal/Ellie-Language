pub const BENCH_CLASS: &str = r#"
/// -- Required Types --
@dont_fix_variant;
class int {}

@dont_fix_variant;
class void {}


@dont_fix_variant;
class bool {}

@dont_fix_variant;
class function {}

@dont_fix_variant;
class string {}

@dont_fix_variant;
class nullAble<T> {}

fn println(s: string);
/// -- Required Types --

class Human  {
	co(name, age);
	v name : string;
	v age : int;
	fn introduce() : string {
		ret "Hello my name is " + self.name + ", I born at " + (2023 - self.age);
	}
}


fn main() {
	v human = new Human("Ahmetcan", 22);
	println(human.introduce());
}
"#;

pub const BENCH_LOOP: &str = r#"
/// -- Required Types --
@dont_fix_variant;
class int {}

@dont_fix_variant;
class void {}


@dont_fix_variant;
class bool {}

@dont_fix_variant;
class function {}

@dont_fix_variant;
class string {}

@dont_fix_variant;
class nullAble<T> {}

fn println(s: string);
/// -- Required Types --

fn main() {
    v last = 0;
    v current = 1;
    v count = 2;
    v fib = 0;

    loop count <= 10 {
        fib = last + current;
        last = current;
        current = fib;
        count += 1;
    }

    println("fib(10) = " + fib);
}
"#;

pub const BENCH_FIBONACCI_LOOP: &str = r#"
/// -- Required Types --
@dont_fix_variant;
class int {}

@dont_fix_variant;
class void {}


@dont_fix_variant;
class bool {}

@dont_fix_variant;
class function {}

@dont_fix_variant;
class string {}

@dont_fix_variant;
class nullAble<T> {}

fn println(s: string);
/// -- Required Types --

fn main() {
    v last = 0;
    v current = 1;
    v count = 2;
    v fib = 0;

    loop count <= 10 {
        fib = last + current;
        last = current;
        current = fib;
        count += 1;
    }

    println("fib(10) = " + fib);
}
"#;

pub const BENCH_FIBONACCI_RECURSION: &str = r#"
/// -- Required Types --
@dont_fix_variant;
class int {}

@dont_fix_variant;
class void {}


@dont_fix_variant;
class bool {}

@dont_fix_variant;
class function {}

@dont_fix_variant;
class string {}

@dont_fix_variant;
class nullAble<T> {}

fn println(s: string);
/// -- Required Types --

fn main() {
    fn fib(n: int) : int {
        if n <= 1 {
            ret n;
        } else {
            ret fib(n - 1) + fib(n - 2);
        }
    }

    println("fib(10) = " + fib(10));
}
"#;
