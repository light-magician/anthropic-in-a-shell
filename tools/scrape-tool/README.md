# getting an LLM to webscrape easily

## basic py setup

make `.venv` based on provided requirements

```bash
python3 -m venv .venv
pip install -r requirements.txt
source .venv/bin/activate
...
pip install whatever
pip freeze > requirements.txt
...
python3 whatever.py
...
deactivate
```
