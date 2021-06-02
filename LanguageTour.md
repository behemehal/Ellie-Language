## Language Tour

* ### Creating variables 
  * Dynamic Variable 
    ```ellie
      d test = 123;
      //Ellie's default integer type is u16
    ```
  * Type Set Variable
    ```ellie
        v test : i16 = 123;
    ```
* Data Types

    - Numeric
        ```rust
            i8
            i16
            i32
            i64
            i128
            isize
            u8
            u16
            u32
            u64
            u128
            usize
            f32
            f64
        ```
    - String
        ```ellie
            v name : string = "ellie";
        ```
    - Char
        ```ellie
            v firstName : char = 'e';
        ```
    - Cloak
        ```ellie
            v letters :  cloak(char, char, char, char, char) = ('e', 'l', 'l', 'i', 'e');
        ```
    - Array
        ```ellie
            v letters : array(char, 5) = [
                'e',
                'l',
                'l',
                'i',
                'e'
            ];
        ```
    - Collective
        
        **Not complete**
        ```ellie
            v letters : collective(i16, char) = {
                1 :'e',
                2 :'l',
                3 :'l',
                4 :'i',
                5 :'e'
            }
        ```
    - Arrow Functions
        ```ellie
            v callingAwesomeness : fn(string, i32)::i8 = @(name: String, count: i8) > string {
                v awesomenes : string;
                for (element, count) {
                    awesomenes += "ellie";
                }
                return awesomeness;
            }
        ```
    - boolean
* Functions
  ```ellie
    fn callingAwesomeness (name: String, count: i8) > string {
        v awesomenes : string;
        //iter count times write to variable i
        for (d i, count) {
            awesomenes += "ellie";
        }
        return awesomeness;
    }

  ```
* Class

    You can see complex [example](./examples/arrayChunker.ei)

    ```ellie
        class Test<T> { //A generic type class


            co Test(a, b) { //Constructor with body

            };

            co Test(a, b) //You can initialize a class without body

            pri v a : T; //A private parameter with generic type
            pri v b : String = "ellie"; //A private parameter with initial data inside

            
            pub v cm : String;


            get test @() > string { //Getter
                return "test";
            }

            set test @(param: String) { //Setter
                self.cm = param; //Changing self member
            }

            pub fn first() { //A public function
                self.second(); //Calling a private function

            }

            pri fn second() { //A private function
            }

        }
    ```
* Enum
    ```ellie
        pub enum Test {
            withValue(i8),
            noValue
        }

        if (value == Test.withValue as data) {
          data //value
        } else if (value == test.noValue) {
          
        }
    ```
* For Loops
    ```ellie
        v reversedEllie : string;
        for (character, ellie_chars.reverse()) { //Array.reverse is standard-library function
            reversedEllie += character;
        }
        reversedEllie //eille
    ```
* Supported Operators
    * Logical & Comp
    ```ellie
    if compare > big {
        //You dont need braces
    } else if compare < small {

    } else if compare < small && compare < big {

    } else if compare < small || compare < big {
    
    } else {

    }
    ```
    * Arithmetic & Assignment
    ```ellie
    d Addition = 1 + 1;
    d Subtraction = 2 - 1;
    d Multiplication = 2 * 2;
    d Exponentiation = 2 ** 3;
    d Division = 4 / 2;
    d Modulus = 5 % 2;

    //Assignment

    v Addition : i16 = 1;
    v Subtraction : i16 = 2;
    v Multiplication : i16 = 2;
    v Exponentiation : i16 = 2;
    v Division : i16 = 4;
    v Modulus : i16 = 5;

    Addition += 1;
    Subtraction -= 1;
    Multiplication *= 2;
    Exponentiation **= 3;
    Division /= 2;
    Modulus %= 2;
  ```


---
##   Here is the good part of our parser
  - Here is the function that parses if else

    https://github.com/behemehal/Ellie-Language/blob/2dbe4cd02f1c14f5931cb8c26d426d9b1dca3b97/parser/src/processors/condition_processor.rs

  - Here is the defination of if else

    https://github.com/behemehal/Ellie-Language/blob/2dbe4cd02f1c14f5931cb8c26d426d9b1dca3b97/parser/src/syntax/condition.rs

Our engine is too flexible for major updates, a person wants to write own element can easily implement it to parser
