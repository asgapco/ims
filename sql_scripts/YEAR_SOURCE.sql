DROP TABLE IF EXISTS YEAR_SOURCE;
CREATE TABLE YEAR_SOURCE(
    CODE VARCHAR(30) PRIMARY KEY,
    ANAME VARCHAR(30),
    ENAME VARCHAR(30),
    FROM_DATE DATE,
    TO_DATE DATE
);

