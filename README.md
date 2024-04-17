# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Subscriber model struct.`
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [x] Commit: `Implement add function in Subscriber repository.`
    -   [x] Commit: `Implement list_all function in Subscriber repository.`
    -   [x] Commit: `Implement delete function in Subscriber repository.`
    -   [x] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [x] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [x] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [x] Commit: `Implement publish function in Program service and Program controller.`
    -   [x] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [x] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?

When it comes to the Observer pattern diagram, the usage of an interface (or trait in rust) for the BambangShio application ultimately lies dependent on the complexity of the system itself. To be more precise, if the application only has a single type of Subscriber, then an interface may not be needed especially since in this case, the Subscribers would have the same methods and behave the same way. On the other hand, if the Subscribers are expected to behave differently it might be wise to consider implementing an interface. This way, it keeps different parts of the system separate from one another, making it more manageable and expandable in the long run.

2. id in Program and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?

When considering using a Vec (list) of a DashMap(map/dictionary), one thing to take into consideration is the uniqueness of the data, the efficiency needed within the system itself, as well as the conccurency of the environment. However, in the case of the BambangShop app, I believe that it is necessary to use the DashMap since uniqueness holds a big requirement for the product id and the url. Aside from that the DashMap also provides efficient lookup operations, ensuring data integrity along the way. Overall, I believe using DashMap would be the better choice in this case, ensuring uniqueness and efficient lookup operations compared with Vec which may not prioritize these necessities.

3. When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?

When programming using Rust, thread-safety really needs to be considered greatly as it is emphasized quite harshly by the language itself. In the case of the List of Subscribers static variable, using a DashMap may be the more suitable approach. The Singleton pattern only has one instance that will exist throughout the enitre lifetime of the program and also only provides a single access point. On the other hand, with a DashMap, it provides thread safety from the start making it a more suitable option in this case. Additionally, DashMap is more widely-used hence promoting reliabilty and safety with its usage.

#### Reflection Publisher-2
1. In the Model-View Controller (MVC) compound pattern, there is no “Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model?

In the Model-View-Controller (MVC) architecture, separating the "Service" and "Repository" from a model aligns with the Single Reponisibility Principle (SRP). By this principle, it is stated that a class should only have one reason to change and in separating the "Service" and "Repository", it allows the model to focus on representing the data itself while the Service would focus in managing and handling the business logic, with the Repository managing the access of data and the storage. With the separation of these components, it promotes a model with more focused components, allowing ease in managing and maintaining in the long run.

2. What happens if we only use the Model? Explain your imagination on how the interactions between each model (Program, Subscriber, Notification) affect the code complexity for each model?

If we do not separate the interactions between each model (Program, Subscriber, Notification), it would most likely cause a significant increase in code complexity. If we were to take into account the idea that each model would have their own separate and individual tasks, without separting them, the code would be overloaded with domain logic, leading to a a very messy codebase. Not to mention, in certain cases where these models would have to take on additional functions and responsibilities, the codebase itself might be too burdened, increasing the likelihood of errors and in return further adding to the overall code complexity of each model.

3. Have you explored more about Postman? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects.

Personally, I found Postman to be a very helpful tool in testing my current work. With the ability to make data easily readable, it helps in checking the accuracy and correctness of my data. Though, I found the request builder interface to be very helpful as it allows me easily create different types of HTTP requests. This specific function may be found very heplful for group projects.

#### Reflection Publisher-3
1. Observer Pattern has two variations: Push model (publisher pushes data to subscribers) and Pull model (subscribers pull data from publisher). In this tutorial case, which variation of Observer Pattern that we use?

In this tutorial case, the Observer Pattern that was used was the Push model which can be seen in the notify method within the NotificationService. This was implemented in the for loop inside the notify method wherein the publisher sends the data to all the subscribed observers when a request occurs.

2.What are the advantages and disadvantages of using the other variation of Observer Pattern for this tutorial case? (example: if you answer Q1 with Push, then imagine if we used Pull)

The Pull Observer Pattern has both its advantages and disadvantages. For exampe, one upside of the pull method is that it can reduce resource consumption since the observers would only receive any updates within the event that they are actively requested. In addition to that, the pull observer pattern could have more control in terms of receving updates as they can specifically choose the timing and frequency of the pulls depending on their specific needs. In contrast to this, a disadvantage with this observer pattern is that it may have a higher latency, resulting in potential delays since the observers themselves must actively interact with the updates. One other disadvantage would be the increase in network traffic in regards to how frequent the observers pull or how many are pulling at the same time.

3. Explain what will happen to the program if we decide to not use multi-threading in the notification process.

If multi-threading was decided not to be used in this program, the notification system could experience clogging. In this system, the notifications are sent through different threads, hence allowing it to be sent sequentially without having the need to wait for the previous notification to be sent first. Without multi-threading, it may cause significant delays to the program, especially if many notifications were to be sent at the same time because it would then have to wait for the previous notification to be sent before the next one.
