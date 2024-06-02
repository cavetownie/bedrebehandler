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

def parse_main() -> None:
    files = get_data_files()
    for file in files:
        try:
            file_data = read_data(file)
            json_data = json.loads(file_data)

            identifier = json_data.get("Identifier")
            postnummer = json_data.get("Postnummer")
            navn = json_data.get("Navn")
            adresse = json_data.get("Adresse")
            beskrivelse = json_data.get("AktuelInformation")
            opdateret = json_data.get("SidstOpdateret")

            if None in (identifier, postnummer, navn, adresse, opdateret):
                logging.warning(f"Skipping file {file}. Containing values which cannot be null according to database schema")
                continue

            if None == beskrivelse:
                beskrivelse = ""
            else:
                info = beskrivelse.lower()
                if "tandlæge" in info:
                    behandler_type = "tandlæge"
                elif "psykolog" in info:
                    behandler_type = "psykolog"
                elif "psykoterapeut" in info:
                    behandler_type = "psykoterapeut"
                elif "øjenlæge" in info:
                    behandler_type = "øjenlæge"
                elif "hudlæge" in info:
                    behandler_type = "hudlæge"
                elif "læge" in info:
                    behandler_type = "læge"
                else:
                    behandler_type = "andet"

            behandler_data = {
                "identifier": int(identifier),
                "postnummer": int(postnummer),
                "kliniktype": behandler_type,
                "navn": navn,
                "adresse": adresse,
                "beskrivelse": beskrivelse,
                "opdateret": opdateret,
            }


            conn = sqlite3.connect(db_path)
            cursor = conn.cursor()

            query = '''
            INSERT INTO behandler (identifier, postnummer, kliniktype, navn, adresse, beskrivelse, opdateret)
            VALUES (:identifier, :postnummer, :kliniktype, :navn, :adresse, :beskrivelse, :opdateret)
            '''

            cursor.execute(query, behandler_data)
            conn.commit()
            conn.close()
        except:
            logging.error(f"Hit unknown error on file {file}")

if __name__ == "__main__":
    db_path = os.path.join(get_path("db"), "bedrebehandler.db")
    data_folder = get_path("data")

    logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
    parse_main()
