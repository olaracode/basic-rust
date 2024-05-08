/*
* Traits allow us to define shared
* behavior between different structs

When creating a trait you declare the name
and its type references(return type, and arguments)
since we don't wanna dictate said method of the trait
will work in a certain case/scenario
!Eg:
pub trait Something {
    fn do_something(&self) -> String
}

If you infact want to implement a default functionality
to the method you can do so by writting the method body
Which can be in turn overwrite in the Struct implementation
!Eg:
pub trait Something {
    fn do_something(&self) -> String {
        String::from("Do Something")
    }
}
*/
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more..)")
    }
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/*
You can also use Traits as parameters
Since traits can be utilized by many struct implementations
we pass as a parameter the &impl TraitName
to specify that any struct that has a summary
implementation can be used

This syntax:
pub fn something(some: &impl Thing){
    ...
}

Is what is called syntax sugar. Meaning something nice to look at

You could also do it by using the default syntax

pub fn something<T: Summary>(item: &T){
    ...
}

Where T is the generic type that is bound to the
Summary trait

The &imple syntax is limited by its simplicity upfront
if we needed to use more than one parameter on a function
and both of them had the same trait bound it would look like this

pub fn something(item: &impl Summary, item2: &impl Summary){
    ...
}

But if we used the trait bound syntax it would look like this

pub fn something<T: Summary>(some: &T, thing: &T){
    ...
}

Which is more readable

But this can change quickly if we use more than one generic type
with more than one Trait for example:
fn something<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U){
    ...
}

In this case for readability purposes we could use the where Clause
fn something<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + debug
{
    ...
}
*/
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("@olaracode"),
        content: String::from("Hello world"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("Octavio Lara"),
        headline: String::from("The sky is falling"),
        content: String::from("The sky is not actually falling"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article Summary: {}", article.summarize());
    notify(&tweet);
    notify(&article);
}
