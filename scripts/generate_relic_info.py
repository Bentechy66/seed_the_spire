import glob
import re

STS_SRC_ROOT = "C:\\Users\\sendb\\Desktop\\Slay the Spire 2\\src\\"
MODEL_ROOT = STS_SRC_ROOT + "Core\\Models\\Relics\\"

relic_names = list()
relic_rarities = list()
relics = list()

for relic in glob.glob(MODEL_ROOT + "*.cs"):
    relic_source = open(relic).read()
    try:
        relic_classname = re.findall(r"class (.+) : RelicModel", relic_source)[0]
        relic_rarity = re.findall(r"override RelicRarity Rarity => RelicRarity\.(.+);", relic_source)[0]
    except:
        continue
    
    relic_names.append(relic_classname)
    relic_rarities.append(relic_rarity)
    relics.append((relic_classname, relic_rarity))

print("#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]")
print("pub enum RelicRarity {")
for rarity in set(relic_rarities):
    print(f"    {rarity},")
print("}")

print("\n\n")

print("#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]")
print("pub enum Relic {")
for name in set(relic_names):
    print(f"    {name},")
print("}")

newline = "\n            "

print(f"""
impl Relic {{
    pub fn rarity(&self) -> RelicRarity {{
        match self {{
            {newline.join(f"Relic::{name} => RelicRarity::{rarity}," for (name, rarity) in relics)}
        }}
    }}
}}""")