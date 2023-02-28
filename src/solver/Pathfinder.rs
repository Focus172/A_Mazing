package code

import objects.Node

class Pathfinder(
    private val startNodeX: Int,
    private val endNodeX: Int,
    private val endNodeY: Int,
    private val walls: Array<BooleanArray>
) {
    protected var open = ArrayList<Node>()
    protected var closed = ArrayList<Node>()
    protected var finalPath = ArrayList<Node>()
    private val startNodeY = 0

    //public int currentX;
    //public int currentY;  
    //TO FIND BASE NODE FIND NODE W/ PARENT "null"
    init {
        //defines instance varibales
        // this may change but for now this will work for ny problems

        //adds start Objects.node to open
        val startNode = Node(startNodeX, startNodeY, null, 0, 0)
        startNode.fCost = calculateFCost(startNode)

        //closeNode(startNode);
    }

    //-----------------------------MAIN MEATHODS--------------------------------
    fun openNode(xIndex: Int, yIndex: Int) {
        //THIS DECLARATION OF PARENT NODE IS WRONG, never mind i think it is right
        open.add(Node(xIndex, yIndex, closed[closed.size - 1], -1, -1)) //this doesnt work if there is nothing in closed
        val current = open[open.size - 1]
        current.fCost = calculateFCost(current)
        current.distanceTraveled = current.parent!!.distanceTraveled + 10
    }

    //when Objects.node is added to closed ajacent is added to open
    fun closeNode(current: Node) {
        closed.add(current)

        //check if current Objects.node is end

        //-----open adjacent if they arnt walls------
        //north
        var newOpenX = current.xIndex
        var newOpenY = current.yIndex - 1
        if (newOpenY > 0 && !walls[newOpenX][newOpenY] && checkIfOpen(newOpenX, newOpenY)) {
            openNode(newOpenX, newOpenY)
        }

        //east
        newOpenX = current.xIndex - 1
        newOpenY = current.yIndex
        if (newOpenX > 0 && !walls[newOpenX][newOpenY] && checkIfOpen(newOpenX, newOpenY)) {
            openNode(newOpenX, newOpenY)
        }

        //south
        newOpenX = current.xIndex
        newOpenY = current.yIndex + 1
        if (newOpenY < 65 && !walls[newOpenX][newOpenY] && checkIfOpen(newOpenX, newOpenY)) {
            openNode(newOpenX, newOpenY)
        }

        //west
        newOpenX = current.xIndex + 1
        newOpenY = current.yIndex
        if (newOpenX < 65 && !walls[newOpenX][newOpenY] && checkIfOpen(newOpenX, newOpenY)) {
            openNode(newOpenX, newOpenY)
        }
        open.remove(current)
    }

    fun solveMaze() {
        //add the correct nodes IN ORDER to the path array
        println("OH Yeah!!!!!!!!!!!!!!!!!!!!!!")
        val overflowCount = 99
        var i = 1
        while (i < overflowCount) {
            val ind = indexOfLowestFCost()
            closeNode(open[ind])
            println(closed[closed.size - 1].xIndex.toString() + " " + closed[closed.size - 1].yIndex)
            println("$endNodeX $endNodeY")
            println("")
            if (closed[closed.size - 1].xIndex == endNodeX && closed[closed.size - 1].yIndex == endNodeY) {
                i = overflowCount
                println("path found!")
            }
            i++
        }
        println("OH NO!!!!!!!!!!!!!!!!!!!!!!")
    }

    //---------------------------------HELPER MEATHODS-------------------------------------
    //works if it is called in very specific curcumstances - change this later
    fun calculateFCost(current: Node): Int {
        var fCost = 0
        if (current.parent != null) fCost += current.parent!!.distanceTraveled

        //find distance to start end
        val loop = true
        var currentXIndex = current.xIndex
        var currentYIndex = current.yIndex

        //find diagonal dis
        while (currentXIndex != endNodeX && currentYIndex != endNodeY) {
            fCost += if (currentXIndex > endNodeX) {
                currentXIndex--
                currentYIndex++
                14
            } else {
                currentXIndex++
                currentYIndex++
                14
            }
        }
        if (currentXIndex == endNodeX) {
            fCost += (currentXIndex - endNodeX) * 10
        } else {
            fCost += Math.abs((currentYIndex - endNodeY) * 10)
        }
        return fCost
    }

    fun indexOfLowestFCost(): Int {
        var indexOfLowestFCost = 0
        for (i in open.indices) {
            if (open[i].fCost < open[indexOfLowestFCost].fCost) {
                indexOfLowestFCost = i
            }
        }
        return indexOfLowestFCost
    }

    private fun checkIfOpen(xIndex: Int, yIndex: Int): Boolean { //there has to be a better way to do this
        for (current in open) { //need to also check closed
            if (current.xIndex == xIndex && current.yIndex == yIndex) {
                return false
            }
        }
        return true
    }
}