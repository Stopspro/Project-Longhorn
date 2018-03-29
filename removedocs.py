# this is a build tool
def process():
  target = raw_input("Filename:")
  docfile = open(target,'r+')
  rust = 'yes'
  curline = 0
  line = f.readline()
  while line:
    curline = curline + 1
     if "//" not in line: # if // is not in line
       rust = 'no'
       python = 'yes'
       #Python
       if "//" not in line # if # is not in line
       python = 'no'
       assembly = 'yes'
       #x86-64 Assembly
         if ";" not in line # if ; is not in line
         assembly = 'no'
     
   if rust = 'yes':
      
    
  
  f.close()
