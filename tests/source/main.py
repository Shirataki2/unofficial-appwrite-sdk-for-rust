import random

def main(req, res):
  payload = req.payload or 'No payload provided. Add custom data when executing function.'

  secretKey = req.env.get(
    'SECRET_KEY',
    'SECRET_KEY environment variable not found. You can set it in Function settings.'
  )

  randomNumber = random.random()

  trigger = req.env['APPWRITE_FUNCTION_TRIGGER']

  return res.json({
    'message': 'Hello from Appwrite!',
    'payload': payload,
    'secretKey': secretKey,
    'randomNumber': randomNumber,
    'trigger': trigger,
  })