from sys import argv
import os
import re

inline = re.compile('\${1}([^$\s]+)\${1}')
display = re.compile('\${2}([^$]+)\${2}')

inline_repl = r'''<img src="{}/{}.png" height=14pt>'''
display_repl = r'''<center>
<img src="{}/{}.png" height=24pt>
</center>'''

template = r'''\documentclass[convert]{{standalone}}
\usepackage{{mathpazo}}
\usepackage{{amsmath}}
\usepackage{{amssymb}}
\begin{{document}}
${}$
\end{{document}}'''

outdir = argv[1]
infile = argv[2]
outfile = argv[3]

with open(infile, 'r') as f:
    contents = ''.join(f.readlines())

try:
    os.mkdir(outdir)
except:
    pass

i = 0

def make_repl(pattern):
    def repl(m):
        global i
        texfile = '{}.tex'.format(i)
        with open(outdir + texfile, 'w') as f:
            f.write(template.format(m.group(1).strip()))
        replacement = pattern.format(outdir, i)
        i += 1
        return replacement
    return repl

contents = inline.sub(make_repl(inline_repl), contents)
contents = display.sub(make_repl(display_repl), contents)

with open(outfile, 'w') as f:
    f.write(contents)

