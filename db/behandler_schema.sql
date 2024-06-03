DROP TABLE IF EXISTS behandler;
DROP TABLE IF EXISTS telefonnumre;

CREATE TABLE behandler (
    identifier integer NOT NULL,
    postnummer integer NOT NULL,
    kliniktype varchar(20) NOT NULL,
    navn varchar(500) NOT NULL,
    adresse varchar(500) NOT NULL,
    beskrivelse varchar(500),
    opdateret date NOT NULL,
    PRIMARY KEY (identifier)
);

CREATE TABLE telefonnumre (
    identifier integer PRIMARY KEY,
    behandler_id integer NOT NULL,
    telefon_nummer varchar(15) NOT NULL, /* (+45) 42424242 */
    str_identifier varchar(500), /* hovednummer */ 
    beskrivelse varchar(500), /* tirsdag 9:00 - 16:45 */ 
    FOREIGN KEY (behandler_id) REFERENCES behandler(identifier) ON DELETE CASCADE
);
