from dao.ds.Tie import Tie

def test_1():
        my_tie = Tie()

        my_tie.add_key("test1")
        my_tie.add_key("foo1")
        my_tie.add_key("foo")
        my_tie.add_key("key")

        assert my_tie.lookup_key("test1")
        assert not my_tie.lookup_key("test2")
        assert my_tie.lookup_key("foo")
        assert my_tie.lookup_key("foo1")
        assert my_tie.lookup_key("key")
        assert not my_tie.lookup_key("pippo")
        assert not my_tie.lookup_key("test")

        assert len(my_tie) == 4

def test_2():
        my_tie = Tie()

        my_tie.add_key("test1")
        my_tie.add_key("test1")
        my_tie.add_key("test2")
        my_tie.add_key("key")

        assert my_tie.lookup_key("test1") 
        assert my_tie.lookup_key("test2")

        assert len(my_tie) == 3

def test_3():
        my_tie = Tie()

        my_tie.add_key("test1")
        my_tie.add_key("key1")

        assert my_tie.lookup_key("test1") 
        assert my_tie.lookup_key("key1")
        assert my_tie.lookup_prefix("test")
        assert my_tie.lookup_prefix("key")
        assert not my_tie.lookup_prefix("foo")