# AWS VPC (Virtual Private Cloud)

- Logically Isolated Section can execute AWS resources in a virtual network that you define yourself.

  - Control over your virtual network resources:
    - Selection of the IP address range;
    - Creation of subnets;
    - Configuration of network gateway route tables.
    
  - Allows you to customize your VPC network configuration.
  - Multiple layers of security.

Resume: a main network and subnets within it, which can be configured.


- Route Table

  - A set of rules or routes that direct network traffic from your subnet.
    - Each route has an origin and a destination;
    - Every table has a local route to communicate within the VPC;
    - Each subnet must be associated with one route table (maximum).

| Destino     | Destino |
| 10.0.0.0/16 | Local   |

Resume: a routing table is like a Waze for the internet.


- Subnets
 
 - Public Subnets: Need to be publicly accessible. Example: a store's website;
 - Private Subnets: Should only be accessible from the private network, such as a database.

Subnets

IP Address and CIDR Block 10.0.0.0/24 | reserved for
10.0.0.0                              | network address
10.0.0.1                              | internal communication
10.0.0.2                              | Domain Name System (DNS) resolution
10.0.0.3                              | future use 
10.0.0.255                            | network broadcast address

VPC 10.0.0.0/16
Subnet 1 (10.0.0.0/24) - 251 Address IP
Subnet 2 (10.0.2.0/24) - 251 Address IP
Subnet 3 (10.0.3.0/24) - 251 Address IP
Subnet 4 (10.0.1.0/24) - 251 Address IP

- Internet Gateway (IGW)

- Allows public internet traffic to your VPC. 

Resume: Acts as a gateway, allowing traffic from the public internet into your VPC.

Route Table - Public

| Destino     | Destino |
| 10.0.0.0/16 | Local   |
| 0.0.0.0/0   | igw-id  |

Internet > IGW > Route Table > Subnets (Public) 

- NAT

NAT - Differing from a gateway, NAT receives incoming data from the internet but doesn't actively send outgoing data to the internet unless it's in response to an incoming request.

Rout Table - Public 

| Destination | Destination |
| 10.0.0.0/16 | Local       |
| 0.0.0.0/0   | igw-id      |

Route Table - Private

| Destination | Destination |
| 10.0.0.0/16 | Local       |
| 0.0.0.0/0   | nat-gwid    |


Example: 

Internet      Masquerade          Desktop (Linux)

Wan           Serve Security      Lan

| <- Package       | <- Package              | <- Package        |
|------------------|-------------------------|-------------------|
| S: 200.100.50.99 | S: 200.100.50.99        | S: 192.168.200.10 |
| D: google.com    | -- S: 192.168.200.10 -- | D: google.com     |
| Dport: 80        | D: google.com           | Dport: 80         |
|                  | Dport: 80               |                   |


| Package ->       | Package ->             |  Package ->       |
|------------------|------------------------|-------------------|
| S: 200.100.50.99 | -- S: 200.100.50.99 -- | S: 192.168.200.10 |
| D: google.com    | S: 192.168.200.10      | D: google.com     |
| Dport: 80        | D: google.com          | Dport: 80         |
|                  | Dport: 80              |                   |


> [!NOTE]  
> Study: AWS Direct Connect VS VPN, CDN - AWM CloudFront


# Create

- VPC

  - Name: vpc-test
  - Block CIDR IPV4 - manual
  - CIDR IPv4: 10.0.0.0/16
  - Block CIDR IPv6 - None
  - Location - standard
  - Tag (Label) - vpc-name
  > Create!


- Subnet

  - Select: vpc-test
  - Name: private-subnet-1a
  - Block CIDR IPv4: 10.0.2.0/24
  > Create


- Subnet

  - Select: vpc-test
  - Name: public-subnet-1a
  - Block CIDR IPv4: 10.0.1.0/24
  > Create


- Route table

  - Edit name: private-route-table


- Internet Gateway (IGW)

  - Name: gateway-test
  - Association (Select): vpc-test 


- Route table
  
  - Select (VPC): vpc-test
  - Edit route: 0.0.0.0/0
  - Select: igw (Gateway internet)


- Subnet 

  - Edit association: public route table


- Gateway NAT

  - Name: gatewaynat-subnet-1a
  - Select: public-subnet-1a
  - Connection type: public
  - IP Elastic Aloocation.
  > Create!
