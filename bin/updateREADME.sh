#!/bin/bash

CURRENT=$(cd $(dirname $0);pwd)
README="$CURRENT/../README.md"

echo '# WriteCodeEveryDay

## Calendar' > $README
cd $CURRENT/.. && node node/gitlog/index.js "./*/*/README.md" | node node/mdCalendar/index.js  >> $README
