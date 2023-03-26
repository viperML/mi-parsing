from typing import Dict
import textract
from pathlib import Path

pdfs_dir = (Path() / "priv").resolve()

pdfs = [file for file in pdfs_dir.glob("**/*.pdf")]
print(pdfs)

pdfs_text: Dict[Path, str] = {f: textract.process(str(f), method="pdfminer").decode() for f in pdfs}

for (f,content) in pdfs_text.items():
    newpath = f.parent / f"{f.name}.txt"
    with open(newpath, "w") as newfile:
        newfile.write(content)
