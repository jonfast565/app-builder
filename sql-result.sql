/*
 * Database Name: AddressValidation
 * Dialect: SqlServer
 */
CREATE DATABASE AddressValidation;
GO
  CREATE TABLE [dbo].[Addresses] (
    [AddressId] BIGINT IDENTITY(1, 1) NOT NULL,
    [AddressLine3] NVARCHAR(MAX) NULL,
    [AddressLine2] NVARCHAR(MAX) NULL,
    [AddressLine1] NVARCHAR(MAX) NOT NULL,
    [City] NVARCHAR(MAX) NOT NULL,
    [State] NVARCHAR(MAX) NOT NULL,
    [ZipCode] NVARCHAR(MAX) NOT NULL,
    [AddressKey] VARBINARY(MAX) NOT NULL
  );
GO
ALTER TABLE
  Addresses
ADD
  CONSTRAINT PK_Addresses PRIMARY KEY ([AddressId]);
GO