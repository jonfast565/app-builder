/*
 * Database Name: LoginDb
 * Dialect: SqlServer
 */
CREATE DATABASE LoginDb;
GO 
CREATE TABLE [dbo].[User] (
        [UserId] BIGINT IDENTITY(1, 1) NOT NULL,
        [AddressLine2] NVARCHAR(MAX) NULL,
        [AddressLine3] NVARCHAR(MAX) NULL,
        [AddressLine1] NVARCHAR(MAX) NOT NULL,
        [City] NVARCHAR(MAX) NOT NULL,
        [State] NVARCHAR(MAX) NOT NULL,
        [ZipCode] NVARCHAR(MAX) NOT NULL,
        [CreatedDateTime] DATETIME2 NOT NULL DEFAULT GETDATE(),
        [CreatedBy] NVARCHAR(MAX) NOT NULL DEFAULT SUSER_SNAME(),
        [UpdatedDateTime] DATETIME2 NOT NULL DEFAULT GETDATE(),
        [UpdatedBy] NVARCHAR(MAX) NOT NULL DEFAULT SUSER_SNAME()
    );
GO 
CREATE TABLE [dbo].[Session] (
        [SessionId] BIGINT IDENTITY(1, 1) NOT NULL,
        [BeginDate] DATETIME2 NULL,
        [EndDate] DATETIME2 NULL,
        [LastActiveDate] DATETIME2 NULL,
        [CreatedDateTime] DATETIME2 NOT NULL DEFAULT GETDATE(),
        [CreatedBy] NVARCHAR(MAX) NOT NULL DEFAULT SUSER_SNAME(),
        [UpdatedDateTime] DATETIME2 NOT NULL DEFAULT GETDATE(),
        [UpdatedBy] NVARCHAR(MAX) NOT NULL DEFAULT SUSER_SNAME()
    );
GO
ALTER TABLE [dbo].[User]
ADD CONSTRAINT PK_User PRIMARY KEY ([UserId]);
GO
ALTER TABLE [dbo].[Session]
ADD CONSTRAINT PK_Session PRIMARY KEY ([SessionId]);
GO