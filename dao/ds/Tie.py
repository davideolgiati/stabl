class Tie():
        class TieNode():
                def __init__(self):
                        self.leafs = {}
                        self.value = False
                
                def add_leaf(self, key):
                        assert isinstance(key, str)
                        assert len(key) == 1

                        if(key not in self.leafs.keys()):
                                self.leafs[key] = Tie.TieNode()

                        assert isinstance(self.leafs[key], Tie.TieNode)
                        return self.leafs[key]           
                
                def get_leaf(self, key):
                        assert isinstance(key, str)
                        assert len(key) == 1

                        if(key in self.leafs.keys()):
                                return self.leafs[key]   
                        else:
                                return None
                
                def get_value(self):
                        return self.value
                
                def set_value(self):
                        self.value = True
        
        def __init__(self):
                self.head = Tie.TieNode()
                self.key_count = 0
        
        def add_key(self, string):
                if self.lookup_key(string):
                        return
                
                currentNode = self.head
                for char in string:
                        currentNode = currentNode.add_leaf(char)
                
                currentNode.set_value()
                self.key_count = self.key_count + 1
        
        def lookup_key(self, string):
                currentNode = self.head
                for char in string:
                        if currentNode is None:
                                break

                        currentNode = currentNode.get_leaf(char)
                        
                
                if(currentNode is not None):
                        return currentNode.get_value()
                else:
                        return False
        
        def __len__(self):
                return self.key_count