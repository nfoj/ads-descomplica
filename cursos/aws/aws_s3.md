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







