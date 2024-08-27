# AWS - Cloud


- Models:

- Tradicional IT
  
  You Manager: 
    - Applications;
    - Data;
    - Runtime;
    - Middleware;
    - Operating System; 
    - Virtualization;
    - Servers; 
    - Storage;
    - Networkin.


- IaaS (Infrastructure as a Service)

  You Manager:
    - Applications;
    - Data;
    - Runtime;
    - Middleware;
  
  Delivered as a service:
    - Operating System; 
    - Virtualization;
    - Servers; 
    - Storage;
    - Networkin.

  - More flexibility regarding network and storage configuration;
  - Manages security aspects;
  - Configures access controls;

  Examples:
    - Aws EC2;
    - Aws Elastic Block Storage (EBS);
    - Aws Virtual Private Cloud (VPC).


- PaaS (Platform as a Service)

  You Manager:
    - Applications;
    - Data;
  
  Delivered as a service:
    - Runtime;
    - Middleware;
    - Operating System; 
    - Virtualization;
    - Servers; 
    - Storage;
    - Networkin.

  - Does not manage underlying infrastructure;
  - AWS manages the operating system: application of database patches, firewall configuration, and disaster recovery;
  - The customer manages the code and data.
  
  Examples:
    - Aws Lambda;
    - Aws Relational Database Service (RDS);
    - Aws Elastic Beanstalk.
    
  
- SaaS (Software as a Service)
  
  Delivered as a service:
    - Applications;
    - Data;
    - Runtime;
    - Middleware;
    - Operating System; 
    - Virtualization;
    - Servers; 
    - Storage;
    - Networkin.
    
  - Centralized hosting
  - Subscription model or pay-per-use
  - Accessed through a browser, mobile app, or API (Application Programming Interface)
  - No need to manage the underlying infrastructure that supports the service 

  Examples:
    - Aws Trusted Advisior;
    - Aws Shield;
    - Aws Chime.


- Cloud

  - Cloud;
  - Hybrid;
  - Private Cloud;

- Classic:
  - Security
    - Firewalls;
    - ACLs;
    - Adm.
      
  - Networks
    - Router; 
    - Network pipeline;
    - Switch.
    
  - CPU
    - Servers Local.

  - Storage
    - DAS;
    - SAN;
    - NAS;
    - RDBMS.


- AWS:
  - Security
    - Security Groups;
    - Network ACLs;
    - IAM.
      
  - Networks
    - Elastic Load Balancing; 
    - Amazon VPC.

  - CPU
    - AMI > AWAC EC2.

  - Storage
    - EBS;
    - EFS;
    - S3;
    - RDS.


- AWS: Shared Responsibility Model
  - Client;
  - AWS.


# Infrastructure

- Region (Availability Zones)

Geographical Area
  - Data replication between regions is controlled by the consumer
  - Communication between regions uses AWS backbone network infrastructure
  
  The region provides full redundancy and connectivity to the network
  The region typically consists of two or more Availability Zones
  
Resume:
 - Region: São Paulo, Virginia, ...
 - Communication exists, but you configure it;
 - Each region is interdependent, they function differently.
 - Each region has a set of data centers.

How to choose a region
 - Data governance: Can data leave the country?
 - Proximity to customers (latency)
 - Services available in the region
 - Costs (vary by region)

An Availability Zone is a discrete location within a region that comprises one or more data centers, designed to provide fault tolerance
↪ and redundancy.

- Elasticity and scalability
- Fault tolerance
- High Availability
  
A single product instance can be replicated across multiple Availability Zones.
