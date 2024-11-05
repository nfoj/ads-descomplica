# AWS - EC2

- Computing Service - Resume

- VMS (Virtual Machine)
  - Amazon EC2
  - Amazon Lightsail

- Containers
  - Amaozn ECS

- PaaS
  - Aws Elastic Beanstalk

- Serverless
  - Aws Lambda
  - Aws Fargate

- Specialized solutions
  - Aws Outposts
  - Aws Batch


- EC2
  - Virtual machines (instances) of ECS in the cloud
  - Full control over the system (Windows or Linux) on each instance
  - Instances of any size
  - Fast and easy execution of instances
  - You need to choose an image
  - You can pay only for what you use


Virtualization
   
   Instance EC2
   |
   Hipervisor
   |
   CPUs
   |
   RAM
   |
   Disk
   |
Server Host


- Configuration parameters
  - Amazon Machine Image (AMI)
  - Type Instance (A1, M4, T3, C5) 
  - Virtual Private Cloud (VPC)
  - Assumed Role
  - User Data
  - Instance Storage or Elastic Block Store (EBS)
  - Security Group
  - Key Par


- AMI - Amazon Machine Image

- Operating System (Windows and Linux)
  - Quick Start - Provided by AWS
  - User createed AMIs 
  - AWS Marketplace
  - Community AMIs

- Positive Points
  - Repeatability
  - Reusability
  - Recoverability

- Create  AMI
  - Painel EC2
  - Run instance
  - Name
  - Selec System
  - Type Instance

- Type Instance
  - What will this be used for? 
    - General purpose (a1, m4, m5, t2, t3)
    - Computational optimization (c4, c5)
    - Memory optmization (r4, r5, x1, z1) - Database
    - Storage optimization (f1, g3, g4, p2, p3) - Machine Learning
    - Accelerated computing (d2, h1, i3) - Distributed file system

- Name

  m5d.xlarge

  - m: family
  - 5: generation
  - d: resources
  - xlarge: size

- Configuration VPC
  - Number of instances
  - Network: VPC
  - Subnet: ...
  ... 

- User date
  - script
    - user text
    - upload file
  
- Storage (Storage + AWS EBS)
  - Root volume
  - Data volume

  Obs: very similar to disk poartitioning settings
  Obs: Read more about AWS EBS


-- 7J -- 
