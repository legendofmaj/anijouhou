#!/bin/bash
# In this example user1 requires an api-key, while user2 does not.
username1="your_username"
api_key_user1="your_api_key"

anijouhou -d
if [[ $1 == "$username1" ]];
then
  anijouhou -u "$1" -k $api_key_user1
else
  anijouhou -u "$1" -k skip
fi