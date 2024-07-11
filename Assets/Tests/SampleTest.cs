using CsBindgen;
using NUnit.Framework;
using UnityEngine;

public class SampleTest
{
    [Test]
    public void SampleTestCreateGame()
    {
        var game = NativeMethods.create_game(3, CellType.O);
        NativeMethods.next(game, new Vec2 { x = 0, y = 0 });
        NativeMethods.next(game, new Vec2 { x = 0, y = 1 });
        NativeMethods.next(game, new Vec2 { x = 1, y = 0 });
        NativeMethods.next(game, new Vec2 { x = 1, y = 1 });
        var status5 = NativeMethods.next(game, new Vec2 { x = 2, y = 0 });
        Debug.Log(status5);
    }

    [Test]
    public void SampleTestBind()
    {
        var game = NativeMethods.create_game(3, CellType.O);
        NativeMethods.bind(game, cell =>
        {
            Debug.Log(cell.cell_type);
            Debug.Log(cell.position.x);
            Debug.Log(cell.position.y);
            Debug.Log(cell.history_count);
        });
        NativeMethods.next(game, new Vec2 { x = 0, y = 0 });
    }

    [Test]
    public void PutSamePosition()
    {
        var game = NativeMethods.create_game(3, CellType.O);
        NativeMethods.next(game, new Vec2 { x = 0, y = 0 });
        var status = NativeMethods.next(game, new Vec2 { x = 0, y = 0 });
        Debug.Log(status);
    }

    [Test]
    public void PutOutOfBound()
    {
        var game = NativeMethods.create_game(3, CellType.O);
        var status = NativeMethods.next(game, new Vec2 { x = 3, y = 3 });
        Debug.Log(status);
    }

    [Test]
    public void PutRandom30Times()
    {
        var game = NativeMethods.create_game(3, CellType.O);
        for (var i = 0; i < 30; i++)
        {
            var status = NativeMethods.next(game, new Vec2 { x = Random.Range(0, 3), y = Random.Range(0, 3) });
            Debug.Log(status);
        }
    }
}