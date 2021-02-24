#!/bin/bash

CURRENT=$(cd $(dirname $0);pwd)
README="$CURRENT/../README.md"

GIT_LOG=$(cd $CURRENT/.. && node node/gitlog/index.js './*/*/README.md')

echo "# Write Code Every Day

## Continuous Commit Recording

$(cd $CURRENT/.. && echo $GIT_LOG | node node/mdRecording/intex.js)

## Commit Calendar

$(cd $CURRENT/.. && echo $GIT_LOG | node node/mdCalendar/index.js)" > $README
