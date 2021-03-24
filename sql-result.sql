/*
 * Database Name: LoginDb
 * Dialect: SqlServer
 */

-- create database
CREATE DATABASE LoginDb;

-- create tables

CREATE TABLE [dbo].[User] (
    
            [UserId]
            
             BIGINT  IDENTITY(1, 1)  
            
            
             NOT NULL 
            
        ,
            [AddressLine2]
             NVARCHAR(MAX) 
            
            
            
             NULL 
            ,
        
            [AddressLine3]
             NVARCHAR(MAX) 
            
            
            
             NULL 
            
        ,
            [AddressLine1]
             NVARCHAR(MAX) 
            
            
            
             NOT NULL 
            ,
        
            [City]
             NVARCHAR(MAX) 
            
            
            
             NOT NULL 
            ,
        
            [State]
             NVARCHAR(MAX) 
            
            
            
             NOT NULL 
            ,
        
            [ZipCode]
             NVARCHAR(MAX) 
            
            
            
             NOT NULL 
            
        
            ,
            [CreatedDateTime] DATETIME2 NOT NULL DEFAULT GETDATE(),
            [CreatedBy] NVARCHAR(MAX) NOT NULL DEFAULT SUSER_SNAME(),
            [UpdatedDateTime] DATETIME2 NOT NULL DEFAULT GETDATE(),
            [UpdatedBy] NVARCHAR(MAX) NOT NULL DEFAULT SUSER_SNAME()
    
);

CREATE TABLE [dbo].[Session] (
    
            [SessionId]
            
             BIGINT  IDENTITY(1, 1)  
            
            
             NOT NULL 
            
        ,
            [BeginDate]
            
            
            
             DATETIME2 
             NULL 
            ,
        
            [EndDate]
            
            
            
             DATETIME2 
             NULL 
            ,
        
            [LastActiveDate]
            
            
            
             DATETIME2 
             NULL 
            
        
            ,
            [CreatedDateTime] DATETIME2 NOT NULL DEFAULT GETDATE(),
            [CreatedBy] NVARCHAR(MAX) NOT NULL DEFAULT SUSER_SNAME(),
            [UpdatedDateTime] DATETIME2 NOT NULL DEFAULT GETDATE(),
            [UpdatedBy] NVARCHAR(MAX) NOT NULL DEFAULT SUSER_SNAME()
    
);


-- add constraints
        
        ALTER TABLE [dbo].[User] ADD CONSTRAINT PK_User 
            PRIMARY KEY ( 
                [UserId]
            );
        
        
        ALTER TABLE [dbo].[Session] ADD CONSTRAINT PK_Session 
            PRIMARY KEY ( 
                [SessionId]
            );
        

-- create relationship tables, columns, and constraints
    CREATE TABLE [dbo].[UserSessionRelationship] (
        
            [UserId] BIGINT REFERENCES [dbo].[User](UserId),
        
            [SessionId] BIGINT REFERENCES [dbo].[Session](SessionId)
        
        PRIMARY KEY ([UserId],[SessionId])
    );
    
