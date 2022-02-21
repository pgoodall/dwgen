#!/usr/bin/python3

# Original command:
# python -c 'import sys, random; L = sys.stdin.readlines(); random.shuffle(L); print "".join(L),'

import sys, random
import os
import sqlite3
import csv
import requests
import colorama
from colorama import Fore
from colorama import Style
import nltkmodules
from nltk.corpus import stopwords
from nltk.tokenize.nist import NISTTokenizer

home = os.path.expanduser("~")
DWGEN_DIR = home + "/.dwgen"
gutenberg_db_file = DWGEN_DIR + "/gutenberg.db"

def create_local_storage():
  try:
    os.mkdir(DWGEN_DIR)
  except OSError:
    print("Failed to create local directory %s " % DWGEN_DIR)
  else:
    print("Created local storage at %s" % DWGEN_DIR)

# Check if the localdir already exists
def setup_local_environment():
  print("Checking if the local storage directory exists at %s ..." % DWGEN_DIR)
  if not os.path.exists(DWGEN_DIR):
    print("Local directory doesn't exist. Creating %s ..." % DWGEN_DIR)
    create_local_storage()
  
  if not os.path.exists(gutenberg_db_file):
    create_gutenberg_db()

# Connect to local db
def setup_storage_db():
  print("Checking for local db ...")
  db_file = DWGEN_DIR + '/db.sqlite3'
  try:
    db = sqlite3.connect(db_file)
  except OSError:
    sys.exit("Failed to create local DB for storage.")

  if not os.path.exists(db_file):
    sys.exit("DB file missing. What happened?")

def build_word_list():
  nist = NISTTokenizer()
  L = sys.stdin.read()
  stop_words = set(stopwords.words('english'))
  words = (nist.tokenize(L, lowercase=True))
  words = [word for word in words if not word in stop_words]
  words = [word for word in words if word.isalpha()]
  words = [word for word in words if len(word) > 3 & len(word) < 9]
  return words

def print_password():
  print("Returning five random words:")
  for count in range(5):
    print(Fore.BLUE + Style.BRIGHT + wordlist[count] + Style.RESET_ALL, end=' ')
  
  print('\n')

def update_guttenberg_db():
  catalog_url = "https://www.gutenberg.org/cache/epub/feeds/pg_catalog.csv"
  with requests.Session() as s:
    catalog = s.get(catalog_url)
    
def create_gutenberg_db():
  create_table = '''CREATE TABLE gutenberg(
                    id INTEGER PRIMARY KEY,
                    format TEXT,
                    pub_date TEXT,
                    title TEXT,
                    lang TEXT,
                    author TEXT,
                    dates TEXT,
                    col_x TEXT,
                    col_y TEXT,
                    col_z TEXT
  );'''
  
  try:
    connection = sqlite3.connect(gutenberg_db_file)
  except OSError:
    sys.exit("Failed to create gutenberg DB.")

  cursor = connection.cursor()
  cursor.execute(create_table)
  

setup_local_environment()
# [ToDo] This should be turned into a function
wordlist = build_word_list()
random.shuffle(wordlist)
print_password()

# This adds and extra empty line and resets the text color to white
# print('\033[1;37;40m \n')
