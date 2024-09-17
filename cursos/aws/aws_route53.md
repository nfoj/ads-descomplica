# Route 53

Route 53 = DNS Service.

- Simple Routing: 
  - is the most basic routing policy, directing all traffic to a single endpoint.
  
- Weighted Round Robin Routing: 
  - allows for distributing traffic across multiple instances with different weights, ensuring a balanced load. 

- Latency Routing: 
  - is ideal for applications that require low latency, such as real-time gaming or video streaming.

- Geolocation Routing: 
  - directs traffic to the geographically closest destination, enhancing the user experience.

- Geoproximity Routing: 
  - routes traffic to the nearest destination based on geographic location and other factors, such as the user's ISP network.

- Failover Routing:
  - ensures high availability by automatically switching to a backup endpoint if the primary endpoint fails.

- Multivalue Answer Routing:
  - provides flexibility in DNS resolution, allowing for more complex routing policies.
