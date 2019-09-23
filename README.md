Basic project to be able to build a full stack application using rust as a backend.

Goal of this app is to be able to manager my meals and receipes:
- Enter my receipes
- Build automatically shopping list with ingredients
- Be able to keep an update list of stuff I have at home to not buy all the time the same stuff
- Keep reccord of how much I paid for each ingredient in which shop
- Build a meal schedule
- Compute the amount of calories of each meal automatically

This is the basic features I'd like to see implemented in it.

Ideally, I'll want to avoid to enter everything manually and have a bot plugged on Telegram (or other) to automate some of this stuff. (taking picture of a receipe in a shop and be able to fill automatically ???)

Front end wise, I am still debating which one I should use - do I go with Vuejs (don't want to struggle with React and all it's dependencies...), or go with some new Rust framework?

API wise, I want to expose all the information though a GraphQL interface - want to have something clean and responsive.

Deployment wise, I don't really care for now. Would like to have each component of the stack in a container and automatically deployed on AWS when if pass the build process.
