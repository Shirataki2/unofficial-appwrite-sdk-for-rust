import random

def main(req, res):
  print(req)

  return res.json({
    'message': 'Hello from Appwrite!',
    'value': random.randint(1, 100),
  })
