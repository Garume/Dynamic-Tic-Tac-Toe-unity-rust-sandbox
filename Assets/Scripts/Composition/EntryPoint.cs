using CsBindgen;
using Presenter;
using UnityEngine;
using View;

namespace Composition
{
    public class EntryPoint : MonoBehaviour
    {
        public void Start()
        {
            var game = NativeMethods.create_game(3, CellType.O);
            var factory = new CellViewFactory();
            var presenter = new GamePresenter(game, factory);
            presenter.Bind();
        }
    }
}