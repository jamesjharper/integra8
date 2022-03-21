# Say Hello world to Integra8.
*Draft publish!*
This crate is *99.9% ready* for non-alpha release! 
Work remaining:
 - [ ] Refactor all crates into single Uber package 
 - [ ] Use tree formatter as default formatter when not explicitly set
 - [ ] Publish to doc.rs

Minimal/No breaking changes are expected moving forward.  

Happy Integra8ing!

```rust
#[macro_use]
pub extern crate integra8;

main_test! {}

#[suite]
mod introducing {
    #[setup]
    fn setup() {
        println!("Get Ready ...");
    }

    #[integration_test]
    fn hello_world_test() {
        println!("Hello integra8!");
    }

    #[teardown]
    fn teardown() {
        println!("Enjoy!");
    }
}


```
## Why Integra8?
Thanks to its thriving community, Rust is increasingly finding more and more uses across the tech stack. With this growth comes the need for new tools to meet its new demands.

Rust has great inbuilt support for Continuous Integration Testing, Integra8's goal is to bring that same experience to the Continuous Deployment side of testing.

You should consider Integra8 for the following use cases
- Web service testing
- Web frontend testing
- Blue/Green Cloud deployments
- Certifications for multiple environments 
- Running many tests at the same time
- Anything with long-running blocking IO

## Why not Integra8?
Integra8 does not aim to replace Rusts existing inbuilt `libtest` framework. `libtest` is great, and many of Integra8's features can be replicated with what's already available in the community. 

## Get Started
Integra8 looks and feels like most other Test frameworks, so getting started should be intuitive after learning some basics.
However, as a framework intended for Continuous Deployment, Integra8 offers a lot more than just tests. 

Check out the getting started guide [here](https://github.com/jamesjharper/integra8/blob/main/examples/README.md)

Happy Integra8ing!