# autorsvp
Rust command-line program to automatically RSVP new Meetup events

# Status update
I still haven't gotten meetup's API to respond and haven't heard back from Meetup. For the sake of completing the project I'm going to make a simple local graphQL server that imitates Meetup's API to finish implementation. I'll be starting on this soon after some classwork obligations.

About the current state of the project:
  - Still learning to use GraphQL but having trouble with certain aspects.
  - Project as-is should demonstrate the general structure however.
  - Project currently sends Meetup's example query of querying the user's own name and ID, and prints the full response via println due to this response currently being an error.
    - I can get this query working within a linux terminal, but something's wrong with the implementation here. The query body should be fine, as sending a query with the exact same body in a linux terminal via curl works properly and gets a correct response.
    - As for why I'm currently doing this query instead of querying a particular group for it's events: I cannot get any GraphQL query that involves parameters to work on Meetup's API. Not even Meetup's example queries that are already pre-written in curl. I've submitted a support request to Meetup to ensure that something isn't wrong.
    - Querying the starwars API with parameters as per Cynic's own examples works even, even within Rust. The main difficulty at this point is understanding Meetup's API.
