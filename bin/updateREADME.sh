#!/bin/bash

CURRENT=$(cd $(dirname $0);pwd)
TARGET_FILE_PATTERN="$CURRENT/../calendar/**/calendar.json"
README="$CURRENT/../README.md"

echo '# WriteCodeEveryDay

## Calendar' > $README
cd $CURRENT/../node/mdCalendar && node index.js $TARGET_FILE_PATTERN >> $README
