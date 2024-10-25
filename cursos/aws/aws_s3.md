# AWS S3

- Stores a large amount of unstructured data;
- Data files are stored as objects within a user-specified bucket;
- The maximum size of a single object is 5 TB;
- All objects have a URL, accessible via REST API (universal namespace);
- Each object possesses a key, version ID, value, metadata, and sub-resources.

Resume: S3 is like a drawer where you can put many different objects.


- Durability: data will not be lost;
- Availability: always available for access;
- Scalability: highly scalable storage;
- Security: multiple layers of security;
- Performance: supports various access patterns;

OBS: S3 provides data redundancy within a region.


Data access

- Private: Only the owner can access.
- Public: Anyone can access.
- Restricted access: The administrator defines who can access.


Access control

- Secret Key: AWS (KMS);
- Server-side encryption: AWS responsibility;
- Client-side encryption: Customer responsibility.


Data reading consistency

- Whenever you modify or delete an object, please allow some time for AWS to update all backups connected to the bucket.


Aws S3 Storage

- S3 standard: frequently accessed data
- S3 standard IA: infrequently accessed long-term data
- S3 One Zone IA: non-critical, infrequently accessed long-term data
- S3 Glacier pr Deep Archive: rarely accessed data


Cicler life

Start =  s3 standart (30 days) > s3 standard - IA (60 days) > s3 one zone IA (90 Days) > delete = finish


# Create Bucket

Name: S3-bucket
Region: choice! (Unique region)
ACLs: recomendation
Access: private
Versioning: disabled
Create


- Events

You can configure your bucket to perform a specific action after a certain event. 

- Create
- Put
- Copy
...

Example: For example, every time a user creates an object, the system can send a notification.


- Permissions

  - Acess file
  - Permissions
  - By default, public acess is blocked > Edit
  - [] Block all acess public
  - Save!

  - Bucket Policy

    - Edit > Code
    or
    - AWS Policy Generator
    
      - Select: S3 Bucket Policy
      - Effect: [X]Allow []Deny
      - Principal: *
      - AWS Service: ...
      - Actions: Select actions ...
      - Amazon Resource Name (ARN): Name File (Search file ARN)      
      - Generate!
      - Copy
      - Paste
      - Save!


- Delete

  - [X]Select file
  - Delete
  - Permanently delete


- Versioning
  
  - Edit versioning bucket
  - [X]Allow
  - Save!
  - Show versions! 


- Control versions

  - Show versions
  - Delete file? [X]Yes 
  
  - Recover file
    - Show versions
    - [X]Select the version with a maker
    - Delete
    - Permanently delete

    - Obs: Each modification or addition of a file with the same name creates a new version, which is appended to the version list. Deleting a version replaces it with the previous one.
      
      - Example:
      
      Tree
      ├── versionId_1  # First version
      │   ├── metadata: { create_in: "2023-11-01" }
      │   └── content: { ... }
      ├── versionId_2  # Second version (... modification)
      │   ├── metadata: { create_in: "2023-11-05" }
      │   └── content: { ... }
      └── versionId_3  # Third version (... modification)
          ├── metadata: { create_in: "2023-11-10" }
          └── content: { ... }
