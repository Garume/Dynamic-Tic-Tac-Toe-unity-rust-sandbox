using System;
using System.Collections.Generic;
using AOT;
using CsBindgen;
using UnityEngine;
using View;

namespace Presenter
{
    public class GamePresenter
    {
        private static readonly List<List<CellView>> Cells = new();
        private readonly CellViewFactory _factory;
        private readonly FfiGame _game;

        public GamePresenter(FfiGame game, CellViewFactory factory)
        {
            _game = game;
            _factory = factory;
        }

        [MonoPInvokeCallback(typeof(NativeMethods.bind_listener_delegate))]
        private static void BindCell(FfiCell cell)
        {
            var cellView = Cells[cell.position.x][cell.position.y];
            switch (cell.cell_type)
            {
                case CellType.X:
                    cellView.ToX();
                    break;
                case CellType.O:
                    cellView.ToO();
                    break;
                case CellType.Empty:
                    cellView.ToEmpty();
                    break;
                default:
                    throw new ArgumentOutOfRangeException();
            }
        }

        public void Bind()
        {
            for (var i = 0; i < 3; i++)
            {
                var row = new List<CellView>();
                for (var j = 0; j < 3; j++)
                {
                    var cell = _factory.Create(new Vector2Int(j, i));
                    cell.onClickAction = () =>
                    {
                        try
                        {
                            NativeMethods.next(_game, new Vec2 { x = cell.position.x, y = cell.position.y });
                        }
                        catch (Exception e)
                        {
                            Debug.Log(e);
                        }


                        Debug.Log("Clicked");
                    };
                    row.Add(cell);
                }

                Cells.Add(row);
            }

            NativeMethods.bind(_game, BindCell);
        }
    }
}