/*
 * Database Name: AddressValidation
 * Dialect: SqlServer
 */
CREATE DATABASE AddressValidation;
GO
  CREATE TABLE [dbo].[Addresses] (
    [AddressId] BIGINT NOT NULL,
    [AddressLine3] NVARCHAR(MAX) NOT NULL,
    [AddressLine2] NVARCHAR(MAX) NOT NULL,
    [AddressLine1] NVARCHAR(MAX) NOT NULL,
    [City] NVARCHAR(MAX) NOT NULL,
    [State] NVARCHAR(MAX) NOT NULL,
    [ZipCode] NVARCHAR(MAX) NOT NULL,
    [AddressKey] NVARCHAR(MAX) NOT NULL,
    [AddressKey] VARBINARY(MAX) NOT NULL
  );
GO