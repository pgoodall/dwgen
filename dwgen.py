#!/usr/bin/python3

# Original command:
# python -c 'import sys, random; L = sys.stdin.readlines(); random.shuffle(L); print "".join(L),'

import sys, random
import os
import sqlite3
from nltk.corpus import stopwords
from nltk.tokenize import word_tokenize

# print("Read in the file from the command line...")
L = sys.stdin.read()
stop_words = set(stopwords.words('english'))
home = os.path.expanduser("~")
localdir = home + "/.dwgen"

def createLocalStorage():
  try:
    os.mkdir(localdir)
  except OSError:
    print("Failed to create local directory %s " % localdir)
  else:
    print("Created local storage at %s" % localdir)

# Check if the localdir already exists
# print("Checking if the local storage directory exists at %s ..." % localdir)
if not os.path.exists(localdir):
  print("Local directory doesn't exist. Creating %s ..." % localdir)
  createLocalStorage()
# else:
#   print("Local directory %s already exists ..." % localdir)

# Connect to local storage
# print("Checking for local db ...")
db_file = localdir + '/db.sqlite3'
try:
  db = sqlite3.connect(db_file)
except OSError:
  sys.exit("Failed to create local DB for storage.")

if not os.path.exists(db_file):
  sys.exit("DB file missing. What happened?")

words = (word_tokenize(L))
words = [word.lower() for word in words if not word in stop_words]
words = [word for word in words if word.isalpha()]
words = [word for word in words if len(word) > 3 & len(word) < 9]

random.shuffle(words)
print("Returning five random words:")
for count in range(5):
  print('\033[1;34;40m %s' % words[count], end = ' ')

print('\033[1;37;40m \n')
