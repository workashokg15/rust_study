/*
The template method pattern is a behavourial design pattern which allows you to define the template, or the skeleton of an operation in a base class, while allowing the subclasses to override specific steps or all steps of the algorithm without changing its structure.

This pattern is very useful when you have a series of steps that needs to be performed in a specific order, but you need different implementations in different situations.

A short explanation:

    The Producer is tasked with producing some product, which in this case can be a Car or a Bike. In order to produce these products, two steps are needed.
    BikeProducer and CarProducer are the two concrete Producers.
    The Client class both the step1 and the step2 method to obtain a product.

Since Rust does not support the notion of superclasses or abstract classes, we can implement this with traits.
Traits: Defining Shared Behavior
A trait defines the functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.
*/

trait Template {
    fn add_frame(&self);
    fn add_wheels(&self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
