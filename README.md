# flight-path

## Setup
Overview: 
- In order to determine the flight path of a person, we must sort through all of their flight records.

Goal: 
- Create a microservice API to track a given person’s flight path. 

Considerations: 
- These flights may not be listed in order and will need to be sorted to find the total flight paths starting and ending airports.

Example Runs: 
- Example 1: 
  ```
  Input:    [['SFO','EWR']]
  Result:   ['SFO', 'EWR']
  Explanation: An individual only took one trip (nonstop flight).
  ```
- Example 2: 
  ```
  Input:    [['ATL', 'EWR'], ['SFO', 'ATL']] 
  Result:   ['SFO', 'EWR']
  Explanation: Once sorted by airport code, you get `[['SFO', 'ATL'], ['ATL', 'EWR']]`.
  ```
- Example 3: 
  ```
  Input:    [['IND', 'EWR'], ['SFO', 'ATL'], ['GSO', 'IND'], ['ATL', 'GSO']]
  Result:   ['SFO', 'EWR']
  Explanation: Once sorted by airport code, you get `[['SFO', 'ATL'], ['ATL', 'GSO'], ['GSO', 'IND'], ['IND', 'EWR']]`.
  ```
## TODO
- [x] Implement in Rust
- [ ] Implement as RESTful microservice with Spring Boot
- [ ] Implement as RESTful microservice with Django
