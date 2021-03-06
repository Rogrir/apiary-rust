-- Add up migration script here
CREATE TABLE ACCOUNT_APIARY (
  ID_account UUID NOT NULL, 
  ID_apiary   int4 NOT NULL, 
  PRIMARY KEY (ID_account, 
  ID_apiary));
ALTER TABLE ACCOUNT_APIARY ADD CONSTRAINT FKAccou_apia926913 FOREIGN KEY (ID_apiary) REFERENCES APIARY (ID);
ALTER TABLE ACCOUNT_APIARY ADD CONSTRAINT FKAccou_apia442043 FOREIGN KEY (ID_account) REFERENCES ACCOUNT (ID);
INSERT INTO ACCOUNT_APIARY (ID_account, ID_apiary) SELECT ID, 1 FROM ACCOUNT WHERE LOGIN = 'admin';
INSERT INTO ACCOUNT_APIARY (ID_account, ID_apiary) SELECT ID, 2 FROM ACCOUNT WHERE LOGIN = 'admin';
INSERT INTO ACCOUNT_APIARY (ID_account, ID_apiary) SELECT ID, 3 FROM ACCOUNT WHERE LOGIN = 'user1';