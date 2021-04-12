import pandas

df = pandas.read_csv("./mod_class_combinations_data.txt")

mod_classes = df.columns.values[2::]

for mc in mod_classes:
    print(f'mod class {mc}')
    m = df[df['atomic'] == True][mc].max()
    print(df[(df[mc] == m) & (df['atomic'] == True)])
