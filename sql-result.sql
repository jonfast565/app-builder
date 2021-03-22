/*
 * Database Name: LoginDb
 * Dialect: SqlServer
 */

CREATE DATABASE LoginDb;
GO


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
            
        
    
);
GO

CREATE TABLE [dbo].[Session] (
    
    
        
            
            [SessionId]
            
             BIGINT  IDENTITY(1, 1)  
            
             NOT NULL 
            
        
    
        
            , 
            [BeginDate]
            
            
            
             NULL 
            
        
            , 
            [EndDate]
            
            
            
             NULL 
            
        
            , 
            [LastActiveDate]
            
            
            
             NULL 
            
        
    
);
GO


    
        
        ALTER TABLE User ADD CONSTRAINT PK_User 
            PRIMARY KEY ( [UserId] );
        
    
GO    

    
        
        ALTER TABLE Session ADD CONSTRAINT PK_Session 
            PRIMARY KEY ( [SessionId] );
        
    
GO    

