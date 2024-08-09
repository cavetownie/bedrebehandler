#!/usr/bin/env python3

import os
import json
import sqlite3
import logging

def get_path(where: str) -> str:
    return os.path.join(os.path.dirname(__file__), "..", where)

def get_data_files() -> list[str]:
    scandir_ctx = os.scandir(os.path.join(data_folder))
    return [entry.name for entry in scandir_ctx if entry.is_file()]

def read_data(filename: str) -> str:
    path = os.path.join(data_folder, filename)
    with open(path, "r") as f:
        data = f.read()

    return data

def add_telefonnummer(identifier, nummer, tekst, beskrivelse):
    query = '''
    INSERT INTO telefonnumre (behandler_id, telefon_nummer, str_identifier, beskrivelse)
    VALUES (:identifier, :nummer, :tekst, :beskrivelse)
    '''

    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    telefon_data = {
        "identifier": identifier,
        "nummer": nummer,
        "tekst": tekst,
        "beskrivelse": beskrivelse,
    }

    cursor.execute(query, telefon_data)
    conn.commit()
    conn.close()


def parse_main() -> None:
    files = get_data_files()

    for file in files:
        file_data = read_data(file)
        json_data = json.loads(file_data)

        """
        // Eksempel
        {"Identifier":10002,"Navn":"Misbrugsbehandling","KlinikType":null,"OrganisationType":19,"Adresse":"Strandparken 48 2. etage","Postnummer":"7900","By":"Nykøbing M","Kommunekode":773,"Regionskode":1081,"PraksisFormTekst":null,"AktuelInformation":"<p>Rusmiddelteam Mors&oslash; henvender sig til alle borgere - uanset alder - som har behov for r&aring;d, vejledning eller behandling i forhold til et alkohol- og stofmisbrug.</p>\n<p>Der tilbydes en bred vifte af tilbud, hvor fokus er, at tage udgangspunkt i den enkelte borgers behandlingsbehov og m&aring;l.<br />\n<br />\n</p>","Tilfredshedsundersoegelse":null,"Email":"","Hjemmeside":"","InformationsKategori":"Andre","InformationsUnderkategori":"Øvrige","InformationsUnderkategorier":null,"UddannerLaeger":null,"AabenForTilgang":null,"Offentliggoeres":true,"SidstOpdateret":"2022-10-12T11:47:59","Lokation":{"Latitude":56.80800529,"Longitude":8.86796782},"Aabningstider":[],"Daekningsomraader":[],"Fravaer":[],"Funktioner":[],"Personale":[],"Produkter":[],"Telefonnumre":[{"Tekst":"Hovednummer","Nummer":"99706263","Beskrivelse":"","Offentligt":true,"TypeId":0}],"Ventetider":[],"Faciliteter":[{"Tekst":"Elevator findes"},{"Tekst":"Handicaptoilet"},{"Tekst":"Handicapvenlig adgang"},{"Tekst":"Mulighed for parkering"}],"Selvbetjening":[],"Adgangsforhold":[],"OevrigeOplysninger":[],"KlinikHierarki":[]}
        """

        identifier = json_data.get("Identifier")
        telefonnumre = json_data.get("Telefonnumre")

        for nummer in telefonnumre:
            if identifier == 4625:
                print(identifier, nummer)
            add_telefonnummer(identifier, nummer["Nummer"], nummer["Tekst"], nummer["Beskrivelse"])

if __name__ == "__main__":
    db_path = os.path.join(get_path("db"), "bedrebehandler.db")
    data_folder = get_path("data")

    logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
    parse_main()
