for f in *.tex; do
    lualatex -shell-escape $f
done
rm *.{aux,log,pdf}
