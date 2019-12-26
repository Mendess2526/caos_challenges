#!/bin/bash


mktable() {
    echo '<html>'
    echo '<head><head/>'
    echo '<body>'
    echo '<table>'

    echo 'group,mean,stddev,user,system,min,max' \
        | sed -e 's/^/<tr><th>/' -e 's|,|</th><th>|g' -e 's|$|</th></tr>|'

    sort -t',' -k2 -n "$1" \
        | sed -e 's/^/<tr><th>/' -e 's|,|</th><th>|g' -e 's|$|</th></tr>|'

    echo '</table>'
    echo '</body>'
    echo '</html>'
}

main() {
    echo '<html>'
    echo '<head><head/>'
    echo '<body>'
    echo '<ul>'
    for date in $(find /home/common/li3-results | grep '\.csv')
    do
        echo -n '<li>'
        mktable "$date" > "$(basename "$date" .csv).html"
        echo -n "<a href='$(basename "$date" .csv).html'>$date</a>"
        echo '</li>'
    done
    echo '</ul>'
    echo '</body>'
    echo '</html>'
}

main > index.html

