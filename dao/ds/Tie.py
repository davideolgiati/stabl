class Tie():
        class TieNode():
                def __init__(self):
                        self.leafs = {}
                
                def addLeaf(self, key):
                        if(key not in self.leafs.keys()):
                                self.leafs[key] = Tie.TieNode()                 
                
                def getLeaf(self, key):
                        if(key in self.leafs.keys()):
                                return self.leafs[key]    
                        else:
                                return None
        
        def __init__(self):
                self._keys = set()
                self.head = Tie.TieNode()
        
        def addValue(self, string, value):
                currentNode = self.head
                for char in string:
                        currentNode.addLeaf(char)
                        currentNode = currentNode.getLeaf(char)
                
                currentNode.value = value
                self._keys.add(string)
        
        def lookupKey(self, string):
                currentNode = self.head
                for char in string:
                        if currentNode is not None:
                                currentNode = currentNode.getLeaf(char)
                
                if(currentNode is not None):
                        return currentNode.value
                else:
                        return None

        def keys(self):
                return list(self._keys)