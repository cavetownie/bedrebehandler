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

def count_tidsbestilling() -> str:
    files = get_data_files()
    v, c = 0, 0
    for file in files:
        try:
            file_data = read_data(file)
            json_data = json.loads(file_data)

            js = json_data.get("Aabningstider")

            if js != None:
                c += 1

            v += 1
        except:
            continue

    return v, c

def add_aabningstid(identifier, dag, aaben, luk):
    query = '''
    INSERT INTO aabningstider (behandler_id, day_of_week, open_time, close_time)
    VALUES (:identifier, :dag, :aaben, :luk)
    '''

    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    aabnings_data = {
        "identifier": identifier,
        "dag": dag,
        "aaben": aaben,
        "luk": luk,
    }

    cursor.execute(query, aabnings_data)
    conn.commit()
    conn.close()


def parse_main() -> None:
    files = get_data_files()

    for file in files:
        try:
            file_data = read_data(file)
            json_data = json.loads(file_data)
            """
            // Eksempel
            "Aabningstider": 
            [{
                "Type":"Åbningstid",
                "Tider": [
                    {
                        "Type":null,
                        "Tekst":"Alle dage",
                        "FraKl":"00:00","TilKl":"24:00",
                        "Bemaerkninger":null
                    }
                ]
            }]
            """

            """
            "Aabningstider": 
            [{
                "Type":"Tidsbestilling",
                "Tider":[
                    {
                        "Type":null,
                        "Tekst":"Man, Tirs, Ons, Tors",
                        "FraKl":"09:30",
                        "TilKl":"12:30",
                        "Bemaerkninger":null
                    },
                    {
                        "Type":null,
                        "Tekst":"Fredag",
                        "FraKl":"10:00",
                        "TilKl":"12:00",
                        "Bemaerkninger":null
                    }
                ]
            }]
            """

            identifier = json_data.get("Identifier")
            aabningstider = json_data.get("Aabningstider")
            for ent in aabningstider:
                if ent["Type"] == "Tidsbestilling":
                    tider = ent["Tider"]

                    # Can contain different times for each day, then there will be
                    # 5 (hverdage) different entries in tider
                    for tid in tider:
                        dage = tid["Tekst"].lower().split(",")
                        for dag in dage:
                            if "man" in dag:
                                add_aabningstid(identifier, 1, tid["FraKl"], tid["TilKl"])
                            if "tir" in dag:
                                add_aabningstid(identifier, 2, tid["FraKl"], tid["TilKl"])
                            if "ons" in dag:
                                add_aabningstid(identifier, 3, tid["FraKl"], tid["TilKl"])
                            if "tor" in dag:
                                add_aabningstid(identifier, 4, tid["FraKl"], tid["TilKl"])
                            if "fre" in dag:
                                add_aabningstid(identifier, 5, tid["FraKl"], tid["TilKl"])

                            # What about satur, sund?
                            # TODO: (?) Add support for multiple opening hours on a day
                            # What happens if open from 9:30 - 10:00 and 11:00 - 16:45 on
                            # a given day?

        except:
            continue

if __name__ == "__main__":
    db_path = os.path.join(get_path("db"), "bedrebehandler.db")
    data_folder = get_path("data")

    logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
    parse_main()
